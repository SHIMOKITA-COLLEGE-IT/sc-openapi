/* eslint-disable no-unused-vars */
const Service = require('./Service');

/**
* Get Generations
* 指定したtypeの`Generation`を全て取得する
*
* type String 
* returns List
* */
const getGenerations = ({ type }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        type,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get Generation
* 指定のgenerationを取得
*
* recordId String 
* returns Generation
* */
const getGenerationsRecordId = ({ recordId }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        recordId,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get Groups
* 指定したtypeの`Group`を全て取得する
*
* type String 
* returns List
* */
const getGroups = ({ type }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        type,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get Group
* 指定のGroupを取得
*
* recordId String 
* returns Group
* */
const getGroupsRecordId = ({ recordId }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        recordId,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get SocialBrands
* Social Brandを全て取得する
*
* returns List
* */
const getSocialBrands = () => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get Users
* 全てのユーザーを取得する
*
* returns List
* */
const getUsers = () => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Get User
* 指定のuserを取得
*
* recordId String 
* returns User
* */
const getUsersRecordId = ({ recordId }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        recordId,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Search Users
* ## TODO  - [ ] フィルタリング - [ ] ページネーション
*
* query String 検索ワード
* returns List
* */
const getUsersSearch = ({ query }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        query,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Login
* ## 概要 accessTokenを取得するためのエンドポイント。  初期登録フォーム未回答の場合、User情報を取得する。  ## レスポンスによるクライアント側の分岐  - 200   - レスポンスにuserがある     - 初期登録フォームに遷移、default値にuserを使用   - レスポンスにuserがない     - Homeに遷移 - 401   - 「このエラーが出たら運営に報告してね」エラーページに遷移 - 403   - 「このアカウントは無効化されてるよ」エラーページに遷移 - 404   - 「まだ運営側でデータ登録が完了していないよ」エラーページに遷移 
*
* loginRequest LoginRequest  (optional)
* returns LoginResponse
* */
const postLogin = ({ loginRequest }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        loginRequest,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);
/**
* Update Me
* Airtable上の自分のUser情報をアップデートする
*
* updateUserRequest UpdateUserRequest  (optional)
* no response value expected for this operation
* */
const putUsers = ({ updateUserRequest }) => new Promise(
  async (resolve, reject) => {
    try {
      resolve(Service.successResponse({
        updateUserRequest,
      }));
    } catch (e) {
      reject(Service.rejectResponse(
        e.message || 'Invalid input',
        e.status || 405,
      ));
    }
  },
);

module.exports = {
  getGenerations,
  getGenerationsRecordId,
  getGroups,
  getGroupsRecordId,
  getSocialBrands,
  getUsers,
  getUsersRecordId,
  getUsersSearch,
  postLogin,
  putUsers,
};
