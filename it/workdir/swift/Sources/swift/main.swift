import Foundation
import Models

while let line = readLine() {
    let json = try? JSONSerialization.jsonObject(
        with: line.data(using: String.Encoding.utf8)!
    )

    let entry = try Test_Entry.decode(json: json as! [String: Any])
    let data = try JSONSerialization.data(withJSONObject: entry.encode())
    let out = String(data: data, encoding: String.Encoding.utf8) as String!
    print(out!)
}
