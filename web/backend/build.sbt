scalaVersion := "2.12.8"

libraryDependencies ++= Seq(
  "com.typesafe.akka" %% "akka-actor" % "2.5.22",
  "com.typesafe.akka" %% "akka-testkit" % "2.5.22" % Test
)

libraryDependencies ++= Seq(
  "com.typesafe.akka" %% "akka-http" % "10.1.8",
  "com.typesafe.akka" %% "akka-http-testkit" % "10.1.8" % Test
)

