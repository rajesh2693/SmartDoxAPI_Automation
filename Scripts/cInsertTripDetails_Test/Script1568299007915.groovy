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
import java.io.File as File

WS.sendRequestAndVerify(findTestObject('aValidate OTP', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid]))

def triPID = UUID.randomUUID().toString().toUpperCase()

println(triPID)

def now = new Date()

tiMe = now.format('dd-MM-YYYY HH:mm:ss +5:30')

println(tiMe)

resp2 = WS.sendRequest(findTestObject('cInsert Trip Detail', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid
            , ('tripid') : triPID, ('startriptime') : tiMe, ('startripLocality') : startripLocality]))

def slurper2 = new groovy.json.JsonSlurper()

def result2 = slurper2.parseText(resp2.getResponseText())

println('***************************************\n')

println('THIS IS THE RESPONSE FROM INSERT TRIP DETAILS  TEST CASE = ' + result2)

println('***************************************\n')

result2.toString().split(triPID, 0)

def value2 = result2.toString().substring(25, 61)

println(value2)

//GlobalVariable.tripid  = value2
println('THE TRIP iD IS   = ' + value2)

f = new File('D:\\InsertTripDetails_ID.txt')

f.write(value2)

