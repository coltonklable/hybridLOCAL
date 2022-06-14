import com.kms.katalon.core.annotation.SetUp
import com.kms.katalon.core.annotation.SetupTestCase
import com.kms.katalon.core.annotation.TearDown
import com.kms.katalon.core.annotation.TearDownTestCase

import groovy.json.JsonOutput
import groovy.json.JsonSlurper
import internal.GlobalVariable

/**
 * Some methods below are samples for using SetUp/TearDown in a test suite.
 */

/**
 * Setup test suite environment.
 */
@SetUp(skipped = false) // Please change skipped to be false to activate this method.
def setUp() {
	// Put your code here.
	def jsonSlurper = new JsonSlurper()
	def searchPayloadFormation = jsonSlurper.parseText(GlobalVariable.Search_Payload_V9)
	GlobalVariable.Search_Payload_V9_Backup = new JsonOutput().toJson(searchPayloadFormation)
}

/**
 * Clean test suites environment.
 */
@TearDown(skipped = true) // Please change skipped to be false to activate this method.
def tearDown() {
	// Put your code here.
}

/**
 * Run before each test case starts.
 */
@SetupTestCase(skipped = true) // Please change skipped to be false to activate this method.
def setupTestCase() {
	// Put your code here.
}

/**
 * Run after each test case ends.
 */
@TearDownTestCase(skipped = false) // Please change skipped to be false to activate this method.
def tearDownTestCase() {
	// Put your code here.
	GlobalVariable.Search_Payload_V9 = GlobalVariable.Search_Payload_V9_Backup
}

/**
 * References:
 * Groovy tutorial page: http://docs.groovy-lang.org/next/html/documentation/
 */