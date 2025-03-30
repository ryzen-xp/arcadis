'use client'

import { useMenu } from '@/context/MenuContext'
import { cn } from '@/lib/utils'

export function LayoutContent({ children }: { children: React.ReactNode }) {
  const { isMenuOpen } = useMenu()

  return (
    <div className={cn(
      "transition-[padding] duration-300 ease-in-out",
      isMenuOpen ? "pt-[280px]" : "pt-20"
    )}>
      {children}
    </div>
  )
} 