<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Retreive PNR</name>
   <tag></tag>
   <elementGuidId>85a35d51-fb59-4797-bc91-c136b9d8ae46</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io:443/uat/${Version}/air/book/reservation/RES_DOMAIN/reservations/${PNR}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Version</defaultValue>
      <description></description>
      <id>4d0f7775-2f51-4747-80ce-6dbac9e1218c</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PNR</defaultValue>
      <description></description>
      <id>0c348aac-a373-4fd5-b96e-e679c4651ff1</id>
      <masked>false</masked>
      <name>PNR</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>b1876821-2655-4db0-9d62-9fdd627c3e40</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>98211df3-9e1e-4095-88b1-6873856e77e6</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>