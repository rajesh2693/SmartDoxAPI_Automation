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

def chkInID = UUID.randomUUID().toString().toUpperCase()

println(chkInID)

f = new File('D:\\SmartDoxApiResponse\\CtID.txt')

f.write(chkInID)


def now = new Date()

chkInTime = now.format('dd-MM-YYYY HH:mm:ss +5:30')

println(chkInTime)

resp3 = WS.sendRequest(findTestObject('bInsert User CheckIn', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') 
	: GlobalVariable.userid , ('checkInTime') : chkInTime, ('checkInID') : chkInID, ('checkinLocality') : checkinLocality]))

def slurper3 = new groovy.json.JsonSlurper()

def result3 = slurper3.parseText(resp3.getResponseText())

println('***************************************\n')

println('THIS IS THE RESPONSE FROM USER_CHECKIN TEST CASE = ' + result3)

println('***************************************\n')

result3.toString().split(chkInID, 0)

def value3 = result3.toString().substring(11,47)

println(value3)

//GlobalVariable.chkInID  = value2

println('THE CHECKIN  iD IS   = ' + value3)

f = new File('D:\\Checkin_ID.txt')

f.write(value3)