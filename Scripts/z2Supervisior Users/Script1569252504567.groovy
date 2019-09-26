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

def ddt = new Date()

fromDatE = ddt.format('dd-MM-YYYY HH:mm:ss +5:30')

println(fromDatE)


def btd = new Date()

toDatE = btd.format('dd-MM-YYYY HH:mm:ss +5:30')

println(toDatE)

WS.sendRequest(findTestObject('y3Supervisior Users', [('uniqueToken') : GlobalVariable.uniquetoken, ('userId') : GlobalVariable.userid
            , ('fromDatE') : fromDatE, ('toDatE') : toDatE]))

responsez2 = WS.sendRequestAndVerify(findTestObject('y3Supervisior Users', [('uniqueToken') : GlobalVariable.uniquetoken
            , ('userId') : GlobalVariable.userid, ('fromDatE') : fromDatE, ('toDatE') : toDatE]))

def slurperz2 = new groovy.json.JsonSlurper()

def resultz2 = slurperz2.parseText(responsez2.getResponseText())

println('***************************************')

println('THIS IS THE RESPONSE FROM SUPERVISIOR USERS TEST CASE = ' + resultz2)

println('***************************************')

def ctID = UUID.randomUUID().toString().toUpperCase()

println(ctID)



f = new File('D:\\SmartDoxApiResponse\\CtID.txt')

f.write(ctID)



def now = new Date()

createdDaTE = now.format('dd-MM-YYYY HH:mm:ss +5:30')

f1 = new File('D:\\SmartDoxApiResponse\\CreatedDate.txt')

f1.write(createdDaTE)



def now1 = new Date()

submittedDaTE = now1.format('dd-MM-YYYY HH:mm:ss +5:30')

f2 = new File('D:\\SmartDoxApiResponse\\SubmitedDate.txt')

f2.write(submittedDaTE)



