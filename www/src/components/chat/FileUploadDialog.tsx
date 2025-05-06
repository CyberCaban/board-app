"use client"

import type React from "react"

import { useRef, useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Checkbox } from "@/components/ui/checkbox"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"

interface FileUploadDialogProps {
  isOpen: boolean
  onOpenChange: (open: boolean) => void
  onUpload: (file: File, fileName: string, isPrivate: boolean) => void
}

export function FileUploadDialog({ isOpen, onOpenChange, onUpload }: FileUploadDialogProps) {
  const [selectedFile, setSelectedFile] = useState<File | null>(null)
  const [fileName, setFileName] = useState("")
  const [isPrivate, setIsPrivate] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const file = e.target.files[0]
      setSelectedFile(file)
      setFileName(file.name)
    }
  }

  const handleUpload = () => {
    if (!selectedFile) return

    onUpload(selectedFile, fileName, isPrivate)

    // Сброс состояния
    setSelectedFile(null)
    setFileName("")
    setIsPrivate(false)
    onOpenChange(false)
  }

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="bg-background text-foreground border-border">
        <DialogHeader>
          <DialogTitle>Upload File</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div className="flex flex-col space-y-2">
            <Button
              variant="outline"
              onClick={() => fileInputRef.current?.click()}
              className="bg-secondary hover:bg-secondary/80 border-border"
            >
              Browse...
              {selectedFile ? ` ${selectedFile.name}` : " No file selected."}
            </Button>
            <input ref={fileInputRef} type="file" onChange={handleFileSelect} className="hidden" accept="image/*" />
          </div>

          <div className="flex flex-col space-y-2">
            <Label htmlFor="filename">Filename</Label>
            <Input
              id="filename"
              value={fileName}
              onChange={(e) => setFileName(e.target.value)}
              className="bg-background border-input"
            />
          </div>

          <div className="flex items-center space-x-2">
            <Checkbox
              id="private"
              checked={isPrivate}
              onCheckedChange={(checked) => setIsPrivate(checked as boolean)}
            />
            <Label htmlFor="private">Private</Label>
          </div>

          <Button onClick={handleUpload} disabled={!selectedFile} className="w-full">
            Upload File
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}

