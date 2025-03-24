"use client"

import type React from "react"

import { useState } from "react"
import { Send } from "lucide-react"
import { useToast } from "@/app/hooks/use-toast"

export default function AwardNominationForm() {
  const { toast } = useToast()
  const [gameName, setGameName] = useState("")
  const [gameDescription, setGameDescription] = useState("")
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [showToast, setShowToast] = useState<{
    visible: boolean
    title: string
    description?: string
    variant?: "default" | "destructive"
  } | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!gameName.trim()) {
      setShowToast({
        visible: true,
        title: "Error",
        description: "Please enter a game name",
        variant: "destructive",
      })

      setTimeout(() => setShowToast(null), 3000)
      return
    }

    setIsSubmitting(true)

    // Simulate API call
    await new Promise((resolve) => setTimeout(resolve, 1000))

    setShowToast({
      visible: true,
      title: "Nomination Submitted",
      description: "Thank you for your nomination!",
    })

    setTimeout(() => setShowToast(null), 3000)

    setGameName("")
    setGameDescription("")
    setIsSubmitting(false)
  }

  return (
    <div className="rounded-lg border border-blue-800/50 bg-blue-950/30 p-6 relative">
      {showToast && showToast.visible && (
        <div
          className={`absolute top-4 right-4 p-4 rounded-md shadow-lg max-w-xs z-50 ${
            showToast.variant === "destructive" ? "bg-red-600" : "bg-green-600"
          }`}
        >
          <h4 className="font-medium text-white">{showToast.title}</h4>
          {showToast.description && <p className="text-sm text-white/90 mt-1">{showToast.description}</p>}
        </div>
      )}

      <h2 className="mb-4 text-xl font-semibold">Submit Your Nomination</h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label htmlFor="game-name" className="mb-2 block text-sm font-medium text-blue-200">
            Game Name
          </label>
          <input
            id="game-name"
            type="text"
            value={gameName}
            onChange={(e) => setGameName(e.target.value)}
            placeholder="Enter the game name"
            className="w-full rounded-md border border-blue-800/50 bg-blue-900/30 px-4 py-2 text-white placeholder:text-blue-400/50 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>

        <div>
          <label htmlFor="game-description" className="mb-2 block text-sm font-medium text-blue-200">
            Why does this game deserve the award? (Optional)
          </label>
          <textarea
            id="game-description"
            value={gameDescription}
            onChange={(e) => setGameDescription(e.target.value)}
            placeholder="Tell us why this game deserves recognition..."
            rows={4}
            className="w-full rounded-md border border-blue-800/50 bg-blue-900/30 px-4 py-2 text-white placeholder:text-blue-400/50 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>

        <button
          type="submit"
          disabled={isSubmitting}
          className="flex w-full items-center justify-center rounded-md bg-blue-500 px-4 py-3 font-medium text-white transition-colors hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-blue-950 disabled:opacity-70"
        >
          {isSubmitting ? (
            <span>Processing...</span>
          ) : (
            <>
              <Send className="mr-2 h-4 w-4" />
              Submit Nomination
            </>
          )}
        </button>
      </form>
    </div>
  )
}

