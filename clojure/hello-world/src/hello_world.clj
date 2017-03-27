(ns hello-world)

(defn say-hello [someone]
  (str "Hello, " someone "!"))

(defn hello
  ([] (say-hello "World"))
  ([someone] (say-hello someone)))
