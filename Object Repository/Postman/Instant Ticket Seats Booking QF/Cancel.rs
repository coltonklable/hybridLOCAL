<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Cancel</name>
   <tag></tag>
   <elementGuidId>ce4ce769-7d5c-43ee-85b7-5cbc5d6df793</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>e2etrackingId</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>traceId</name>
      <type>Main</type>
      <value>cancel</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${ntac_environment}/pp/END_TRANSACT_DOMAIN/V${version}/reservations/${eCacheIdQfCancel}/offers/${offerIdQfCancel}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ntac_environment</defaultValue>
      <description></description>
      <id>9061d2be-ef47-4b7c-849c-4c6ee209bec8</id>
      <masked>false</masked>
      <name>ntac_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>2dc1c2f3-6e72-4637-b5ed-2c5714a0b38b</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.eCacheIdQfCancel</defaultValue>
      <description></description>
      <id>64470727-60af-47c1-8df7-a8896a020f67</id>
      <masked>false</masked>
      <name>eCacheIdQfCancel</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.offerIdQfCancel</defaultValue>
      <description></description>
      <id>c2709ee0-b2e9-424f-8ad8-9536ecf25598</id>
      <masked>false</masked>
      <name>offerIdQfCancel</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>956e5ab9-287e-4337-b073-2c2ffa9bd43c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>71403641-3bcc-400b-aa25-3ac5a8d84b2b</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>f2b1097e-1e38-4546-abd2-37f7866e9dd5</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
