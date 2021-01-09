description = "Ballista protocol buffer format"

plugins {
    java
    id("com.google.protobuf") version "0.8.11"
    id("idea")
}

repositories {
    mavenLocal()
    mavenCentral()
    jcenter()
}

sourceSets {
    main {
        proto {
            srcDir("../../proto")
        }
    }
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:3.11.4")
}
