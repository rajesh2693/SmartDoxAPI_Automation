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

WS.sendRequest(findTestObject('y1Mobile App Version', [('uniqueToken') : GlobalVariable.uniquetoken, ('oparatingSystem') : oparatingSystem
            , ('versionNumber') : versionNumber]))

responsez = WS.sendRequestAndVerify(findTestObject('y1Mobile App Version', [('uniqueToken') : GlobalVariable.uniquetoken, ('oparatingSystem') : oparatingSystem
            , ('versionNumber') : versionNumber]))


def slurperz = new groovy.json.JsonSlurper()

def resultz = slurperz.parseText(responsez.getResponseText())

println('***************************************')

println('THIS IS THE RESPONSE FROM MOBILE APP VERSION TEST CASE = ' + resultz)

println('***************************************')


//******************************************************************************




def CtID = UUID.randomUUID().toString().toUpperCase()

f = new File('D:\\SmartDoxApiResponse\\CtID.txt')

f.write(CtID)




def now = new Date()

CreatedDate = now.format('dd-MM-YYYY HH:mm:ss +5:30')

f = new File('D:\\SmartDoxApiResponse\\CreatedDate.txt')

f.write(CreatedDate)



def now1 = new Date()

SubmitedDate = now1.format('dd-MM-YYYY HH:mm:ss +5:30')

f = new File('D:\\SmartDoxApiResponse\\SubmitedDate.txt')

f.write(SubmitedDate)

