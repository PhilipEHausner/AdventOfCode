plugins {
    kotlin("jvm") version "1.8.21"
    application
}

group = "org.example"
version = "1.0-SNAPSHOT"
val korimVersion = "4.0.2"

repositories {
    mavenCentral()
}

dependencies {
    // For JVM only
    implementation("com.soywiz.korlibs.korim:korim-jvm:$korimVersion")
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(11)
}

application {
    mainClass.set("MainKt")
}