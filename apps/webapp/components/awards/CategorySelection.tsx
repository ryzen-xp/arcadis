"use client"

import type React from "react"

import { useState } from "react"
import { Trophy, Lightbulb, Palette, Cpu, Users } from "lucide-react"
import { cn } from "@/lib/utils"

type Category = {
  id: string
  name: string
  description: string
  icon: React.ReactNode
}

const categories: Category[] = [
  {
    id: "game-of-the-year",
    name: "Game of the Year",
    description: "The most outstanding game that pushed boundaries and captivated players",
    icon: <Trophy className="h-5 w-5" />,
  },
  {
    id: "innovation-excellence",
    name: "Innovation Excellence",
    description: "Games that introduced groundbreaking mechanics or technology",
    icon: <Lightbulb className="h-5 w-5" />,
  },
  {
    id: "best-visual-design",
    name: "Best Visual Design",
    description: "Excellence in artistic direction and visual presentation",
    icon: <Palette className="h-5 w-5" />,
  },
  {
    id: "technical-achievement",
    name: "Technical Achievement",
    description: "Outstanding technical implementation and performance",
    icon: <Cpu className="h-5 w-5" />,
  },
  {
    id: "community-choice",
    name: "Community Choice",
    description: "Games that have made the biggest impact on the community",
    icon: <Users className="h-5 w-5" />,
  },
]

export default function CategorySelection() {
  const [selectedCategory, setSelectedCategory] = useState<string>("game-of-the-year")

  return (
    <div>
      <h2 className="mb-4 text-xl font-semibold">Categories</h2>
      <div className="space-y-3">
        {categories.map((category) => (
          <button
            key={category.id}
            onClick={() => setSelectedCategory(category.id)}
            className={cn(
              "w-full rounded-lg border border-blue-800/50 p-4 text-left transition-all",
              selectedCategory === category.id
                ? "bg-blue-900/50 border-blue-500/50 shadow-[0_0_15px_rgba(59,130,246,0.15)]"
                : "bg-blue-950/30 hover:bg-blue-900/30",
            )}
          >
            <div className="flex items-start gap-3">
              <div
                className={cn(
                  "flex h-8 w-8 shrink-0 items-center justify-center rounded-md",
                  selectedCategory === category.id ? "bg-blue-500 text-white" : "bg-blue-900/50 text-blue-300",
                )}
              >
                {category.icon}
              </div>
              <div>
                <h3 className="font-medium text-white">{category.name}</h3>
                <p className="mt-1 text-sm text-blue-200/70">{category.description}</p>
              </div>
            </div>
          </button>
        ))}
      </div>
    </div>
  )
}

