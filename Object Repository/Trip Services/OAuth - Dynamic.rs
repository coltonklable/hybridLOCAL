<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OAuth - Dynamic</name>
   <tag></tag>
   <elementGuidId>092df6e8-2fa8-437d-b1f8-5c2bfc569010</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;grant_type&quot;,
      &quot;value&quot;: &quot;password&quot;
    },
    {
      &quot;name&quot;: &quot;username&quot;,
      &quot;value&quot;: &quot;${OauthUsername}&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;${OauthPassword}&quot;
    },
    {
      &quot;name&quot;: &quot;client_id&quot;,
      &quot;value&quot;: &quot;${OauthClientId}&quot;
    },
    {
      &quot;name&quot;: &quot;client_secret&quot;,
      &quot;value&quot;: &quot;${OauthClientSecret}&quot;
    },
    {
      &quot;name&quot;: &quot;scope&quot;,
      &quot;value&quot;: &quot;openid&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://oauth.qa.travelport.com/oauth/oauth20/token</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.OauthUsername</defaultValue>
      <description></description>
      <id>4e5d51fe-edd4-4ef3-a810-cdb2394ae51f</id>
      <masked>false</masked>
      <name>OauthUsername</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.OauthPassword</defaultValue>
      <description></description>
      <id>d7ea01be-67e6-4f6b-bb18-626f0f175359</id>
      <masked>false</masked>
      <name>OauthPassword</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.OauthClientId</defaultValue>
      <description></description>
      <id>35ec09f4-5744-4ed9-8874-0b312af51c79</id>
      <masked>false</masked>
      <name>OauthClientId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.OauthClientSecret</defaultValue>
      <description></description>
      <id>561fb657-bba3-463a-a775-80576200904b</id>
      <masked>false</masked>
      <name>OauthClientSecret</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def jsonSlurper = new JsonSlurper()
def jsonResponse = jsonSlurper.parseText(response.getResponseText())

GlobalVariable.token = jsonResponse.access_token;</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
