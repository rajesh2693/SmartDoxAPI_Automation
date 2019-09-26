import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

File file3 = new File('D:\\Checkin_ID.txt')

def checkinID = file3.readLines()

println('-------------->' + checkinID)

def now1 = new Date()

chkOutTime = now1.format('dd-MM-YYYY HH:mm:ss +5:30')

println(chkOutTime)

resp4 = WS.sendRequest(findTestObject('bUpdate User CheckIn', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') 
	: GlobalVariable.userid, ('checkinID') : checkinID, ('checkedOutTime') : chkOutTime]))



