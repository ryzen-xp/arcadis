import type { Metadata } from "next"
import CategorySelection from "@/components/awards/CategorySelection"
import AwardNominationForm from "@/components/awards/AwardNominationForm"

export const metadata: Metadata = {
  title: "Award Nominations | Game Awards",
  description: "Nominate your favorite games for Arcadis awards",
}

export default function AwardsPage() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-[#050e29] to-[#0a2550] text-white">
      {/* Navigation */}
      <header className="border-b border-blue-900/30 bg-[#050e29]/90 py-4">
        <div className="container mx-auto flex items-center justify-between px-4">
          <nav className="flex items-center space-x-6">
            <a href="/" className="text-sm font-medium text-white/90 hover:text-white">
              Home
            </a>
            <a href="/games" className="text-sm font-medium text-white/90 hover:text-white">
              Games
            </a>
            <a href="/news" className="text-sm font-medium text-white/90 hover:text-white">
              News
            </a>
            <a href="/community" className="text-sm font-medium text-white/90 hover:text-white">
              Community
            </a>
            <a href="/awards" className="text-sm font-medium text-blue-400 hover:text-blue-300">
              Awards
            </a>
          </nav>
          <button className="rounded-full bg-blue-500 px-6 py-1.5 text-sm font-medium text-white hover:bg-blue-600">
            Connect
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-12">
        <div className="mb-8 text-center">
          <h1 className="mb-2 text-4xl font-bold text-blue-400 md:text-5xl">Award Nominations</h1>
          <p className="text-lg text-blue-100/70">Nominate your favorite games for Arcadis awards</p>
        </div>

        <div className="mt-12 grid gap-8 lg:grid-cols-[1fr,1.2fr]">
          <CategorySelection />
          <AwardNominationForm />
        </div>
      </main>

      {/* Footer */}
      <footer className="mt-16 border-t border-blue-900/30 py-8 text-center text-sm text-blue-100/50">
        <div className="container mx-auto px-4">
          <p className="mb-6 max-w-2xl mx-auto">
            Welcome to the future of gaming, by gamers, for gamers © 2025 • Arcadis All related content, characters,
            names and materials that could be part of an existing work, are the exclusive property of their authors.
          </p>

          <div className="mb-6 flex justify-center space-x-6">
            <a href="#" aria-label="Discord">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-blue-300/70 hover:text-blue-300"
              >
                <circle cx="9" cy="12" r="1" />
                <circle cx="15" cy="12" r="1" />
                <path d="M7.5 7.5c3.5-1 5.5-1 9 0" />
                <path d="M7.5 16.5c3.5 1 5.5 1 9 0" />
                <path d="M15.5 17c0 1 1.5 3 2 3 1.5 0 2.833-1.667 3.5-3 .667-1.667.5-5.833-1.5-11.5-1.457-1.015-3-1.34-4.5-1.5l-1 2.5" />
                <path d="M8.5 17c0 1-1.356 3-1.832 3-1.429 0-2.698-1.667-3.333-3-.635-1.667-.476-5.833 1.428-11.5C6.151 4.485 7.545 4.16 9 4l1 2.5" />
              </svg>
            </a>
            <a href="#" aria-label="Twitter">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-blue-300/70 hover:text-blue-300"
              >
                <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" />
              </svg>
            </a>
            <a href="#" aria-label="YouTube">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-blue-300/70 hover:text-blue-300"
              >
                <path d="M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17" />
                <path d="m10 15 5-3-5-3z" />
              </svg>
            </a>
            <a href="#" aria-label="Twitch">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-blue-300/70 hover:text-blue-300"
              >
                <path d="M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7" />
              </svg>
            </a>
            <a href="#" aria-label="LinkedIn">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-blue-300/70 hover:text-blue-300"
              >
                <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
                <rect width="4" height="12" x="2" y="9" />
                <circle cx="4" cy="4" r="2" />
              </svg>
            </a>
            <a href="#" aria-label="Telegram">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-blue-300/70 hover:text-blue-300"
              >
                <path d="m22 2-7 20-4-9-9-4Z" />
                <path d="M22 2 11 13" />
              </svg>
            </a>
          </div>

          <div className="flex justify-center space-x-6 text-xs">
            <a href="#" className="text-blue-300/70 hover:text-blue-300">
              Privacy Policy
            </a>
            <a href="#" className="text-blue-300/70 hover:text-blue-300">
              Terms Of Service
            </a>
            <a href="#" className="text-blue-300/70 hover:text-blue-300">
              Register
            </a>
          </div>
        </div>
      </footer>
    </div>
  )
}

