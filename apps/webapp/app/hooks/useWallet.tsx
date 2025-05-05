"use client";

import { useWalletContext } from "../context/walletContext";
import {
  WalletNetwork,
  ISupportedWallet
} from "@creit.tech/stellar-wallets-kit";
import { useState } from "react";
import { kit } from "../constants/wallet.constants";
import { toast } from 'sonner';

export const useWallet = () => {
  const walletState = useWalletContext();
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const connectWallet = async () => {
    try {
      setIsConnecting(true);
      setError(null);

      await kit.openModal({
        modalTitle: "Connect Stellar Wallet",
        notAvailableText: "Wallet not available",
        onWalletSelected: async (option: ISupportedWallet) => {
          try {
            kit.setWallet(option.id);
            const { address } = await kit.getAddress();

            const walletName = option.name || "Unknown Wallet";

            walletState.connect(address, walletName);
            return { success: true, address };
          } catch (error) {
            const errorMessage = (error as Error)?.message || "Error connecting wallet";
            setError(errorMessage);
            console.error("Error connecting wallet:", error);
            return { success: false, error: errorMessage };
          } finally {
            setIsConnecting(false);
          }
        },
        onClosed: (err) => {
            if (err) {
              if (err.message === "Modal closed") {
                console.log("User closed the wallet selection modal");
              } else {
                setError(err.message);
                console.error("Modal closed with error:", err);
                toast.error("Error connecting wallet: " + err.message);
              }
            }
            setIsConnecting(false);
          }
      });

      return { success: true };
    } catch (error: unknown) {
      const errorMessage = (error as Error)?.message || "Error connecting wallet";
      setError(errorMessage);
      console.error("Error opening wallet modal:", error);
      return { success: false, error: errorMessage };
    } finally {
      setIsConnecting(false);
    }
  };

  const disconnectWallet = async () => {
    try {
      setError(null);
      await kit.disconnect();
      walletState.disconnect();
      return { success: true };
    } catch (error: unknown) {
      const errorMessage = (error as Error)?.message || "Error disconnecting wallet";
      setError(errorMessage);
      console.error("Error disconnecting wallet:", error);
      return { success: false, error: errorMessage };
    }
  };

  const signTransaction = async (xdr: string) => {
    try {
      setError(null);
      if (!walletState.address) {
        throw new Error("No wallet connected");
      }

      const { signedTxXdr } = await kit.signTransaction(xdr, {
        address: walletState.address,
        networkPassphrase: WalletNetwork.TESTNET,
      });

      return { success: true, signedTxXdr };
    } catch (error: unknown) {
      const errorMessage = (error as Error)?.message || "Error signing transaction";
      setError(errorMessage);
      console.error("Error signing transaction:", error);
      return { success: false, error: errorMessage };
    }
  };

  return {
    connectWallet,
    disconnectWallet,
    signTransaction,
    isConnecting,
    error,
    isConnected: walletState.connected,
    walletAddress: walletState.address,
    walletName: walletState.name,
  };
};
