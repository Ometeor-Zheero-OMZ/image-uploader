"use client";

import axios from "axios";
import { useState } from "react";

interface UploadResult {
  file: string;
  original_size: string;
  optimized_size: string;
  original_bytes: number;
  optimized_bytes: number;
  bytes_saved: number;
}

const FileUpload = () => {
  const [files, setFiles] = useState<FileList | null>(null);
  const [results, setResults] = useState<UploadResult[]>([]);

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFiles(event.target.files);
    }
  };

  const handleDrop = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    if (event.dataTransfer.files) {
      setFiles(event.dataTransfer.files);
    }
  };

  const handleDragOver = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
  };

  const handleUpload = async () => {
    if (files) {
      const formData = new FormData();
      Array.from(files).forEach((file) => {
        formData.append("files", file);
      });

      const response = await axios.post(
        "http://localhost:8080/api/upload",
        formData
      );

      console.log(response.data);
      setResults(response.data);
    }
  };

  console.log("Upload Results:", results);

  return (
    <div>
      <h1>ファイルアップロード</h1>

      <input type="file" multiple onChange={handleFileSelect} />

      <div
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        style={{
          border: "2px dashed #ccc",
          padding: "20px",
          marginTop: "20px",
          textAlign: "center",
        }}
      >
        <p>ここにファイルをドラッグ＆ドロップ</p>
      </div>

      {results.length > 0 && (
        <div>
          <h2>処理結果</h2>
          <table>
            <thead>
              <tr>
                <th>ファイル名</th>
                <th>最適化前サイズ</th>
                <th>最適化後サイズ</th>
                <th>最適化前バイト数</th>
                <th>最適化後バイト数</th>
                <th>削減バイト数</th>
              </tr>
            </thead>
            <tbody>
              {results.map((result, index) => (
                <tr key={index}>
                  <td>{result.file || "不明なファイル"}</td>
                  <td>{result.original_size || "不明"}</td>
                  <td>{result.optimized_size || "不明"}</td>
                  <td>{(result.original_bytes || 0).toLocaleString()} bytes</td>
                  <td>
                    {(result.optimized_bytes || 0).toLocaleString()} bytes
                  </td>
                  <td>
                    {(result.bytes_saved || 0).toLocaleString()} bytes
                    {result.bytes_saved > 0 ? " 🎉" : " ⚠️"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <div>
        {files && (
          <ul>
            {Array.from(files).map((file, index) => (
              <li key={index}>
                {file.name} - {file.size.toLocaleString()} bytes
              </li>
            ))}
          </ul>
        )}
      </div>
      <button
        className="bg-blue-500 rounded p-3 shadow-sm text-white hover:bg-blue-600"
        onClick={handleUpload}
      >
        ファイルをアップロード
      </button>
    </div>
  );
};

export default FileUpload;
